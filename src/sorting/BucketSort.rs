pub fn bucket_sort<Hi, Fi, Ti>(arr: &mut [Ti], hasher: Fi)

where
	Hi: Ord,
	Fi: Fn(&Ti) -> Hi,
	Ti: Ord + Clone,{

let mut buckets: Vec<Bucket<Hi, Ti>> = Vec::new();

	for value in arr.iter() {
	    let hash = hasher(&value);
	    let value = value.clone();
	    match buckets.binary_search_by(|bucket| bucket.hash.cmp(&hash)) {
	        Ok(index) => buckets[index].values.push(value),
	        Err(index) => buckets.insert(index, Bucket::new(hash, value)),
	    }
	}

	let ret = buckets
	    .into_iter()
	    .flat_map(|mut bucket| {
	        bucket.values.sort(); 
	        bucket.values
	    })
	    .collect::<Vec<Ti>>();

	arr.clone_from_slice(&ret);
}

struct Bucket<Hi, Ti> {
	hash: Hi,
	values: Vec<Ti>,
}

impl<Hi, Ti> Bucket<Hi, Ti> {

	pub fn new(hash: Hi, value: Ti) -> Bucket<Hi, Ti> {
	    Bucket {
	        hash,
	        values: vec![value],
	    }
	}
}

#[cfg(test)]

mod base {
	use super::*;
	fn bucket_sort_(arr: &mut [i32]) {
	    bucket_sort(arr, |int| int / 4);
	}
	base_cases!(bucket_sort_);
}

#[cfg(test)]

mod stability {
	use super::*;
	fn bucket_sort_(arr: &mut [(i32, i32)]) {
	    bucket_sort(arr, |t| t.0 / 4);
	}
	stability_cases!(bucket_sort_);
}
