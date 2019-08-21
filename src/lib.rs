pub struct ToyVec<T> {
    elements: Box<[T]>, // T型の要素を格納する領域。各要素はヒープ領域に置かれる
    len: usize,         // ベクタの長さ（現在の要素数）
}

// トレイト境界としてDefaultを設定する
impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size) // T型のデフォルト値をsize個作り
            .collect::<Vec<_>>() // Vec<T>に格納
            .into_boxed_slice() // Box<[T]>に変換
    }

    // ベクタの長さを返す
    pub fn len(&self) -> usize {
        self.len
    }

    // ベクタの現在のキャパシティを返す
    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            // 要素を追加するスペースがないので、大きいelementを確保し、既存の要素を引っ越す
            self.grow();
        }
        self.elements[self.len] = element;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            // インデックスが範囲内ならSome(不変の参照)を返す
            Some(&self.elements[index])
        } else {
            // 範囲外ならNoneを返す
            None
        }
    }

    fn grow(&mut self) {
        // self.capacityが0のときは、allocate_in_heap(1)で長さ1のBox<[T]>を作成し、self.elementsにセットする
        // self.capacityが1以上のときは、allocate_in_heap(self.capacity() * 2)で現在の2倍の長さのBox<[T]>を生成し、self.elementsにセットする。
        // 既存の全要素を新しいBox<[T]>へムーブしたあと、古いBox<[T]>を破棄する
        unimplemented!()
    }
}
