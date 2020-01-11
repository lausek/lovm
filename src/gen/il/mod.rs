use super::*;

// home of the lovm intermediate language
//
// TODO: new ast structure
// TODO: lalrpop to parse file
//
// example code:
//
// def conv($from) -> i64
//  return $from@i64 // as call
// end
//
// def fun($x: i64) -> i64
//  return 0
// end
//
// def fun($x)
//  return 1
// end
//
// def fib($x) -> i64
//  if $x == 0 or $x == 1
//      return 1
//  else
//      return fib($x-1) + fib($x-2)
//  end
// end
//
// def goto_example()
// label1:
//  goto label1
// end
