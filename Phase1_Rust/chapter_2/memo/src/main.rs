struct Counter {
    value: i32,
}

impl Counter {
    // 【関連関数】：selfを取らない。コンストラクタとして使う。
    fn zero() -> Counter {
        Counter { value: 0 }
    }

    // 【メソッド（参照）】：&selfを取る。値を読み取るだけ。
    fn get(&self) -> i32 {
        self.value
    }

    // 【メソッド（可変参照）】：&mut selfを取る。値を変更する。
    fn increment(&mut self) {
        self.value += 1;
    }
}

fn main() {
    // 1. 関連関数の呼び出し（型名::関数名）
    // インスタンスがまだ無いので Counter:: を使う
    let mut c = Counter::zero();

    // 2. メソッドの呼び出し（インスタンス.メソッド名）
    // すでに存在するインスタンス `c` に対してドットで呼び出す
    println!("初期値: {}", c.get()); // 0

    c.increment();
    println!("インクリメント後: {}", c.get()); // 1
}
