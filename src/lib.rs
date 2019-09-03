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

    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
        self.get(index).unwrap_or(default)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // 要素の値をデフォルト値と置き換え、要素の値を取得する
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elem)
        }
    }

    fn grow(&mut self) {
        // 既存の全要素を新しいBox<[T]>へムーブしたあと、古いBox<[T]>を破棄する
        if self.capacity() == 0 {
            // self.capacityが0のときは、allocate_in_heap(1)で長さ1のBox<[T]>を作成し、self.elementsにセットする
            self.elements = Self::allocate_in_heap(1);
        } else {
            // self.capacityが1以上のときは、allocate_in_heap(self.capacity() * 2)で現在の2倍の長さのBox<[T]>を生成し、self.elementsにセットする。
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }
}

// ライフタイムの指定により、このイテレータ自身またはnext()で得た &'vec T型の値が生存している間はToyVecは変更できない
pub struct Iter<'vec, T> {
    elements: &'vec Box<[T]>, // ToyVec構造体のelementsを指す不変の参照
    len: usize,               // ToyVecの長さ
    pos: usize,               // 次に返す要素のインデックス
}

impl<T: Default> ToyVec<T> {
    pub fn iter<'vec>(&'vec self) -> Iter<'vec, T> {
        Iter {
            elements: &self.elements,
            len: self.len,
            pos: 0,
        }
    }
}

impl<'vec, T> Iterator for Iter<'vec, T> {
    // 関数型でこのイテレータがイテレートする要素の型を指定する
    type Item = &'vec T;

    // nextメソッドは次の要素を返す
    // 要素があるなら不変の参照(&T)をSomeで包んで返し、ないときはNoneを返す
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}
