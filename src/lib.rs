#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub lucene);


struct Test {
	pub input: &'static str,
	pub output: &'static str,
}

impl Test {
	pub fn run(self) -> bool {
		let sql = lucene::ExprParser::new().parse(self.input).unwrap();
		println!("Expected: {}\nGot:      {}", self.output, sql);
		sql == self.output
	}
}

#[test]
fn lucene() {
	let tests = vec![
		Test {
			input: "((ee:\"val\" OR ee:*toma) AND NOT p:[* TO pepe]) AND a:(what OR no AND >=2) OR c:(>=awesome OR <excellent)",
			output: "((ee = 'val' OR ee like '%toma') AND NOT p <= 'pepe') AND (a = 'what' OR a = 'no' AND a >= 2) OR (c >= 'awesome' OR c < 'excellent')",
		},
		Test {
			input: "a:b",
			output: "a = 'b'",
		},
		Test {
			input: "a:(b OR c)",
			output: "(a = 'b' OR a = 'c')",
		},
		Test {
			input: "a:(1 OR c)",
			output: "(a = 1 OR a = 'c')",
		},
		Test {
			input: "a:(bc* OR d)",
			output: "(a like 'bc%' OR a = 'd')",
		},
		Test {
			input: "a:(b OR c) AND date:[\"2019-02-20\" TO \"2019-02-25\"]",
			output: "(a = 'b' OR a = 'c') AND date BETWEEN '2019-02-20' AND '2019-02-25'",
		},
		Test {
			input: "a:(b OR NOT c)",
			output: "(a = 'b' OR NOT a = 'c')",
		},
		Test {
			input: "date:[* TO \"2019-02-25\"]",
			output: "date <= '2019-02-25'",
		},
		Test {
			input: "date:[\"2019-02-25\" TO *]",
			output: "date >= '2019-02-25'",
		},
		Test {
			input: "date:{\"2019-02-20\" TO \"2019-02-25\"}",
			output: "date > '2019-02-20' AND date < '2019-02-25'",
		},
	];

	for test in tests.into_iter() {
		assert!(test.run());
	}
}
