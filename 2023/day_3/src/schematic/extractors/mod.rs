use super::line::SchematicLine;
use super::number_container::LineNumberContainer;

pub mod gear_ratio_extractor;
pub mod part_number_extractor;

pub trait ItemExtractor {
    fn items_in_line(&self) -> LineNumberContainer;
}