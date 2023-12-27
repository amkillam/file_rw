#[macro_export] macro_rules! for_each_preprocessor
{
    ($file_reader_or_writer:ident, |$preprocessor:ident| $block:block) => {
        
            let mut $preprocessor = $file_reader_or_writer.preprocess_with::<CharIndexMatrix>();
            $block
            let mut $preprocessor = $file_reader_or_writer.preprocess_with::<ContinuousHashmap>();
            $block
            let mut $preprocessor = $file_reader_or_writer.preprocess_with::<WindowsHashmap>();
            $block
            let mut $preprocessor = $file_reader_or_writer.preprocess();
            $block
        
    };
}