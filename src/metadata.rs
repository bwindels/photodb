extern crate rexiv2;

use self::rexiv2::Metadata;

#[derive(Debug)]
pub struct PhotoMetadata {
	lat: f32,
	lon: f32
}

pub fn parse_metadata(buffer: &[u8]) -> Option<PhotoMetadata> {
	Metadata::new_from_buffer(buffer).map(|parser| {
		let gps_info = parser.get_gps_info();
		gps_info.map(|gi| {
			PhotoMetadata {
				lat: gi.latitude as f32,
				lon: gi.longitude as f32
			}
		})
	}).unwrap_or(None)
}

