use super::section::SectionCode;
use nom::{
    IResult,
    bytes::complete::tag,
    number::complete::{le_u8, le_u32},
    sequence::pair,
};
use nom_leb128::leb128_u32;
use num_traits::FromPrimitive as _;

#[derive(Debug, PartialEq, Eq)]
pub struct Module {
    pub magic: String,
    pub version: u32,
}

impl Default for Module {
    fn default() -> Self {
        Self {
            magic: "\0asm".to_string(),
            version: 1,
        }
    }
}

impl Module {
    pub fn new(input: &[u8]) -> anyhow::Result<Self> {
        let (_, module) =
            Self::decode(input).map_err(|e| anyhow::anyhow!("failed to parse wasm: {}", e))?;
        Ok(module)
    }

    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, _) = tag("\0asm")(input)?;
        let (input, version) = le_u32(input)?;

        let module = Module {
            magic: "\0asm".to_string(),
            version,
        };

        Ok((input, module))
    }
}

fn decode_section_header(input: &[u8]) -> IResult<&[u8], (SectionCode, u32)> {
    let (input, (code, size)) = pair(le_u8, leb128_u32)(input)?; // ①
    
    Ok((
        input,
        (
            SectionCode::from_u8(code).expect("unexpected section code"), // ②
            size,
        ),
    ))
}

#[cfg(test)]
mod tests {
    use crate::binary::module::Module;
    use anyhow::Result;

    #[test]
    fn decode_simplest_module() -> Result<()> {
        // プリアンブルしか存在しないwasmバイナリを生成
        let wasm = wat::parse_str("(module)")?;
        // バイナリをデコードしてModule構造体を生成
        let module = Module::new(&wasm)?;
        // 生成したModule構造体が想定通りになっているかを比較
        assert_eq!(module, Module::default());
        Ok(())
    }
}
