use std::fmt::Debug;
use std::fmt;

/// ジェネリックなスタック構造体です。
///
/// `Stack` は要素のジェネリックなスタックを表します。`T` はスタックに格納できる要素の型です。
pub struct Stack<T> {
    /// スタックの現在の位置を示すポインタです。
    pointer: usize,
    /// スタックの容量を表します。
    capacity: usize,
    /// スタックに格納されたデータのベクタです。
    data: Vec<T>
}


/// スタック操作中に発生する可能性があるエラーを表す列挙型です。
///
/// `StackError` はスタック操作中に起こりうるエラーを表します。主にスタックがオーバーフローした場合や、
/// スタックが空の状態で要素を取り出そうとした場合に発生します。
#[derive(Debug, PartialEq)]
pub enum StackError {
     /// スタックがオーバーフローした場合のエラー
    Overflow,
    /// スタックが空の状態で要素を取り出そうとした場合のエラー
    Underflow,
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackError::Overflow => write!(f, "Stack overflow error"),
            StackError::Underflow => write!(f, "Stack underflow error"),
        }
    }
}

impl<T: Default+ Clone + Debug> fmt::Debug for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Stack {{ capacity: {}, pointer: {}, data: {:?} }}",
            self.capacity,
            self.pointer,
            self.data.iter().take(self.pointer).cloned().collect::<Vec<_>>(),
        )
    }
}

impl<T: Default+ Clone + Debug> Stack<T>  {
    /// 新しいスタックを作成します。指定されたサイズで初期化し、デフォルト値で埋めます。
    ///
    /// # Arguments
    ///
    /// * `size` - スタックの容量
    ///
    /// # Examples
    ///
    /// ```
    /// let stack: Stack<i32> = Stack::new(10);
    /// ```
    pub fn new(size: usize) -> Stack<T> {
        Stack {
            pointer: 0,
            capacity: size,
            data: vec![T::default(); size]
        }
    }

    /// スタックの現在のサイズを返します。
    ///
    /// # Examples
    ///
    /// ```
    /// let stack: Stack<i32> = Stack::new(10);
    /// assert_eq!(stack.size(), 0);
    /// ```
    pub fn size(&self) -> usize {
        self.pointer
    }

    /// スタックが空かどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// let stack: Stack<i32> = Stack::new(10);
    /// assert!(stack.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.pointer == 0
    }

    /// スタックのトップにある要素を返します。スタックが空の場合は None を返します。
    ///
    /// # Examples
    ///
    /// ```
    /// let mut stack: Stack<i32> = Stack::new(10);
    /// stack.push(42).unwrap();
    /// assert_eq!(stack.top(), Some(42));
    /// ```
    pub fn top(&self) -> Option<T>{
        if self.is_empty() {
            None
        }else{
            Some(self.data[self.pointer-1].clone())
        }
    }

    /// スタックに新しい要素を追加します。スタックが満杯の場合はエラーを返します。
    ///
    /// # Arguments
    ///
    /// * `value` - 追加する要素の値
    ///
    /// # Examples
    ///
    /// ```
    /// let mut stack: Stack<i32> = Stack::new(2);
    /// stack.push(42).unwrap();
    /// assert_eq!(stack.size(), 1);
    /// ```
    pub fn push(&mut self, value : T) -> Result<(), StackError> {
        if self.pointer >= self.capacity {
            return Err(StackError::Overflow);
        }

        self.data[self.pointer] = value;
        self.pointer += 1;
        Ok(())
    }  


    /// スタックから要素を取り出します。スタックが空の場合はエラーを返します。
    ///
    /// # Examples
    ///
    /// ```
    /// let mut stack: Stack<i32> = Stack::new(2);
    /// stack.push(42).unwrap();
    /// let popped_value = stack.pop().unwrap();
    /// assert_eq!(popped_value, 42);
    /// ```
    pub fn pop(&mut self) -> Result<T, StackError>{
        if self.pointer == 0 {
            return Err(StackError::Underflow);
        }

        let value = self.data[self.pointer-1].clone();
        self.pointer -= 1;
        Ok(value)
    } 
}