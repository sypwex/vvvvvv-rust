// This header holds Maths functions that emulate the functionality of flash's

// //random
// //Returns 0..1
// float inline fRandom(void)
// {
//     return ( float(rand()) / float(RAND_MAX)) ;
// }

pub fn clamp(x: i32, a: i32, b: i32) -> i32 {
  // x < a ? a : (x > b ? b : x)
  if x < a {
    a
  } else {
    if x > b {
      b
    } else {
      x
    }
  }
}

// struct point
// {
//     int x;
//     int y;
// };

// inline int VVV_min(const int a, const int b)
// {
//     if (a < b)
//     {
//         return a;
//     }
//     else
//     {
//         return b;
//     }
// }

// inline int VVV_max(const int a, const int b)
// {
//     if (a > b)
//     {
//         return a;
//     }
//     else
//     {
//         return b;
//     }
// }
