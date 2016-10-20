use std::path::Path;

extern crate csv;

extern crate generic_array;
use generic_array::GenericArray;

extern crate rustc_serialize;
use rustc_serialize::{Decodable, Decoder};

#[derive(Clone, Debug)]
pub struct DataPoint<N>(GenericArray<f64, N>)
    where N: generic_array::ArrayLength<f64>;


impl <N> DataPoint<N>
    where N: generic_array::ArrayLength<f64>
{
    pub fn squared_euclidean_distance(&self, other: &DataPoint<N>) -> f64
        where N: generic_array::ArrayLength<f64>
    {
        let iter = self.0.iter().zip(other.0.iter());
        iter.fold(0.0, |acc, x| acc + (x.0 - x.1).powi(2))
    }
}


impl <N: Default+Copy+Decodable> Decodable for DataPoint<N>
    where N: generic_array::ArrayLength<f64>
{
    fn decode<S: Decoder>(decoder: &mut S) -> Result<DataPoint<N>, S::Error> {
        decoder.read_seq(|decoder, _| {
            let mut arr  = GenericArray::<f64, N>::new();
            for (i, val) in arr.iter_mut().enumerate() {
                *val = try!(decoder.read_seq_elt(i, Decodable::decode));
            }
            Ok(DataPoint(arr))
        })
    }
}


/// Structure for holding data point's assignments to clusters
#[derive(Clone, Debug)]
pub struct Assignment<'a, N>
    where N: generic_array::ArrayLength<f64> + 'a
{
    data_point: &'a DataPoint<N>,
    cluster_ind: usize,
}


pub fn read_data<P, N>(file_path: P) -> Vec<DataPoint<N>>
    where P: AsRef<Path>,
    N: generic_array::ArrayLength<f64> + std::marker::Copy + std::default::Default + rustc_serialize::Decodable
{
    let mut reader = csv::Reader::from_file(file_path).unwrap();
    reader.decode().map(|point| point.unwrap()).collect()
}


pub fn index_of_min_val<I>(floats: I) -> Option<usize>
    where I: IntoIterator<Item = f64>,
{
    let mut iter = floats.into_iter()
                         .enumerate();

    iter.next()
        .map(|(i, min)| {
            iter.fold((i, min), |(min_i, min_val), (i, val)| {
                if val < min_val { (i, val) }
                else { (min_i, min_val) }
            }).0
        })
}


/// Assign points to clusters
pub fn expectation<'a, N>(data: &'a [DataPoint<N>],
                   cluster_centroids: &[DataPoint<N>]) -> Vec<Assignment<'a, N>>
    where N: generic_array::ArrayLength<f64>
{
    data.iter().map(|point| {
        let distances = cluster_centroids.iter()
                                         .map(|cluster| point.squared_euclidean_distance(cluster));
        let index = index_of_min_val(distances).expect("No minimum value found");
        Assignment { data_point: point, cluster_ind: index }
    }).collect()
}
