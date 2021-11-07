use thiserror::Error;

#[cfg(any(feature = "transforms", feature = "dataset", feature = "mix_utils"))]
pub use crate::rng::*;

type Result<T> = std::result::Result<T, UtilsError>;

#[derive(Error, Debug)]
pub enum UtilsError {
    #[error("NaN detected")]
    NaN,
}

pub(crate) struct NonNan(f32);

impl NonNan {
    fn new(val: f32) -> Option<NonNan> {
        if val.is_nan() {
            None
        } else {
            Some(NonNan(val))
        }
    }
    fn get(&self) -> f32 {
        self.0
    }
}

pub(crate) fn find_max<'a, I>(vals: I) -> Result<f32>
where
    I: IntoIterator<Item = &'a f32>,
{
    vals.into_iter().try_fold(0., |acc, v| {
        let nonnan: NonNan = match NonNan::new(*v) {
            None => return Err(UtilsError::NaN),
            Some(x) => x,
        };
        Ok(nonnan.get().max(acc))
    })
}

pub(crate) fn find_max_abs<'a, I>(vals: I) -> Result<f32>
where
    I: IntoIterator<Item = &'a f32>,
{
    vals.into_iter().try_fold(0., |acc, v| {
        let nonnan: NonNan = match NonNan::new(v.abs()) {
            None => return Err(UtilsError::NaN),
            Some(x) => x,
        };
        Ok(nonnan.get().max(acc))
    })
}

pub(crate) fn find_min<'a, I>(vals: I) -> Result<f32>
where
    I: IntoIterator<Item = &'a f32>,
{
    vals.into_iter().try_fold(0., |acc, v| {
        let nonnan: NonNan = match NonNan::new(*v) {
            None => return Err(UtilsError::NaN),
            Some(x) => x,
        };
        Ok(nonnan.get().min(acc))
    })
}

pub(crate) fn find_min_abs<'a, I>(vals: I) -> Result<f32>
where
    I: IntoIterator<Item = &'a f32>,
{
    vals.into_iter().try_fold(0., |acc, v| {
        let nonnan: NonNan = match NonNan::new(v.abs()) {
            None => return Err(UtilsError::NaN),
            Some(x) => x,
        };
        Ok(nonnan.get().min(acc))
    })
}

pub(crate) fn argmax<'a, I>(vals: I) -> Result<usize>
where
    I: IntoIterator<Item = &'a f32>,
{
    let mut index = 0;
    let mut high = std::f32::MIN;
    vals.into_iter().enumerate().for_each(|(i, v)| {
        if v > &high {
            high = *v;
            index = i;
        }
    });
    Ok(index)
}

pub(crate) fn argmax_abs<'a, I>(vals: I) -> Result<usize>
where
    I: IntoIterator<Item = &'a f32>,
{
    let mut index = 0;
    let mut high = std::f32::MIN;
    vals.into_iter().enumerate().for_each(|(i, v)| {
        if v > &high {
            high = v.abs();
            index = i;
        }
    });
    Ok(index)
}

#[test]
fn test_find_max_abs() -> Result<()> {
    let mut x = vec![vec![0f32; 10]; 1];
    x[0][2] = 3f32;
    x[0][5] = -10f32;
    let max = find_max_abs(x.iter().flatten())?;
    assert_eq!(max, 10.);
    Ok(())
}
