macro_rules! ifelse {
  ($c:expr, $v:expr, $v1:expr) => {
    if $c {
      $v
    } else {
      $v1
    }
  };
}

macro_rules! extend {
  ($v:expr, $v1:expr, $c:expr) => {
    $v = format!("{}{}", $v, format!($v1, $c))
  };
}

pub(crate) use extend;
pub(crate) use ifelse;
