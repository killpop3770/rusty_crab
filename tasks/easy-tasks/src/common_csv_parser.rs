use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
    path::PathBuf,
};

#[derive(Debug)]
pub struct CSVRecord {
    items: Vec<String>,
}

pub struct CommonCSVParser {
    file_path: PathBuf,
}

impl CommonCSVParser {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    pub fn iter(&self) -> Result<CommonCSVParserIterator, Error> {
        let file = File::open(&self.file_path)?;
        let buf_reader = BufReader::new(file);
        Ok(CommonCSVParserIterator { buf_reader })
    }
}

pub struct CommonCSVParserIterator {
    buf_reader: BufReader<File>,
}

impl CommonCSVParserIterator {
    fn new(buf_reader: BufReader<File>) -> Self {
        Self { buf_reader }
    }
}

impl Iterator for CommonCSVParserIterator {
    type Item = CSVRecord;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.buf_reader.by_ref().lines().next()?.ok()?;
        let items = line
            .split(',')
            .map(|a| a.to_string())
            .collect::<Vec<String>>();
        Some(CSVRecord { items })
    }
}
