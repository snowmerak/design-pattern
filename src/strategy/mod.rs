
pub trait Compressor {
    fn compress(&self, data: String) -> String;
    fn decompress(&self, data: String) -> String;
}

pub struct Gzip;

impl Compressor for Gzip {
    fn compress(&self, data: String) -> String {
        format!("{} (gzipped)", data)
    }

    fn decompress(&self, data: String) -> String {
        match data.strip_suffix(" (gzipped)") {
            Some(data) => data.to_string(),
            None => data,
        }
    }
}

pub struct Brotli;

impl Compressor for Brotli {
    fn compress(&self, data: String) -> String {
        format!("{} (brotli)", data)
    }

    fn decompress(&self, data: String) -> String {
        match data.strip_suffix(" (brotli)") {
            Some(data) => data.to_string(),
            None => data,
        }
    }
}

pub struct Snappy;

impl Compressor for Snappy {
    fn compress(&self, data: String) -> String {
        format!("{} (snappy)", data)
    }

    fn decompress(&self, data: String) -> String {
        match data.strip_suffix(" (snappy)") {
            Some(data) => data.to_string(),
            None => data,
        }
    }
}

pub struct ResponseWriter {
    compressor: Box<dyn Compressor>,
}

impl ResponseWriter {
    pub fn new(compressor: Box<dyn Compressor>) -> ResponseWriter {
        ResponseWriter { compressor }
    }

    pub fn write_response(&self, data: String) -> String {
        self.compressor.compress(data)
    }

    pub fn set_compressor(&mut self, compressor: Box<dyn Compressor>) {
        self.compressor = compressor;
    }
}

pub fn example() {
    let mut writer = ResponseWriter::new(Box::new(Gzip));
    let data = "Hello, World!".to_string();
    let compressed = writer.write_response(data.clone());
    println!("{}", compressed);

    writer.set_compressor(Box::new(Brotli));
    let compressed = writer.write_response(data.clone());
    println!("{}", compressed);

    writer.set_compressor(Box::new(Snappy));
    let compressed = writer.write_response(data.clone());
    println!("{}", compressed);
}