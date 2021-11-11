/* practice11 relock by rust(cargo)
 * 		written by Matsumoto Kazuki
 *
 *
 *
 */


use std::sync::RwLock;

mod modtest
{
	extern crate lazy_static;
	use lazy_static::*;
	use std::sync::RwLock;

	lazy_static!														//関数などを使うstaticの初期化のため
	{
		static ref GLOCK :RwLock<ForTest> = RwLock::new(ForTest::new(&5_usize, &5_usize));
																		//なぜかmutではなくref
	}

	struct ForTest
	{
		x :u32,
		y :u32,
	}
	impl ForTest
	{
		fn new(x :&usize, y :&usize) -> Self
		{
			Self
			{
				x :*x as u32,
				y :*y as u32,
			}
		}
	}

	pub fn rewrite_x(x :&u32)
	{
		let mut for_test = GLOCK.write().unwrap();

		for_test.x = *x;
	}

	pub fn return_x() -> u32
	{
		let for_test = &GLOCK.read().unwrap();

		for_test.x
	}
}


fn main()
{

	let lock = RwLock::new(5);

	// many reader locks can be held at once
	{
	    let r1 = lock.read().unwrap();
	    let r2 = lock.read().unwrap();
	    assert_eq!(*r1, 5);
	    assert_eq!(*r2, 5);
	} // read locks are dropped at this point

	// only one write lock may be held, however
	{
	    let mut w = lock.write().unwrap();
	    *w += 1;
	    assert_eq!(*w, 6);
	} // write lock is dropped here

	println!("default x : { }", modtest::return_x());
	modtest::rewrite_x(&10_u32);
	println!("current x : { }", modtest::return_x());

}