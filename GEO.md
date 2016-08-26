On of the features I want in photodb is to be able to search for all photos in a certain geographical area, like a city or country.

A JPEG can contain a latitude and a longitude in the EXIF tags, so that's what we would use.

To know whether a (lat,long) is inside a given city our country, we'll need a database of geographical boundries by name. MapZen has build a dataset for the whole world based on OSM maps. The compressed geojson is 1,2 GB but we can create a storage format that is a lot smaller and should be able to keep that dataset under 200mb.




