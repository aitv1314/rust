fn add_function(){
    println!("add function");
}

fn add(x1:i32, x2:i32)->i32{
let x = x1 + x2;
x
}

fn if_else(){
    println!("function with if and else");

    let x = 12;
    if x > 5 {
        println!("so big");
    }else {
        println!("so small");
    }

    let y = true;
    if y {
        println!("is true");
    }else{
        println!("is false");
    }
}

fn match_test(){
    let x = 1234;
    match x{
        1=>println!("1"),
        _=>println!("{}",x),
    }
}

fn test_if(){
    let nu = if 4>5 {
        55-2
    }else{
        33-200
    };

    println!("{}",nu);
}

fn loop_test(){
    let mut count = 0;
    let res = loop{
        count+=1;
        println!("777");
        if count == 10{
            break count // have ; or not are all fine
        }
    };

    println!("count = {}", res);
}

fn test_while(){
    let arr = [1,2,3,6,10,4,7,33];
    let mut le = arr.len();
    while le > 0{
        le -= 1;
        println!("{} ", arr[le]);
    }
}

fn test_for(){
    let arr = [55,22,8,9,9,9,9,10];

    for a in arr.iter(){
        println!("{}", a);
    }
}

fn test_range(){
    for i in(1..5).rev(){
        println!("{}", i);
    }
}

fn test_owner(){
    /*
    0. 栈上的数据必须拥有固定的大小，堆上的数据编译时大小未知或者大小可能变化
    1. 每一个值都有一个对应的变量作为其所有者
    2. 同一时间值只有一个所有者
    3. 所有者离开自己的作用域时，其所有的值就会被释放掉
    4. 变量离开作用域时，会调用一个叫做drop的特殊函数
    5. 作用域结束的地方自动调用drop函数（即c++中的RAII）

    caution：
        rust中 {}表示一个作用域或者隔离一个作用域
    */
    println!("test owner");
}

fn test_string(){
    let s1 = "hello world";
    let s2 = s1; // 复制，s1中还有值

    let s3 = String::from("hello world x"); // 堆上分配，大小可变；长度、容量、指针分配在栈上，字符串内容分配在堆上
    let s4 = s3; // 此时s2为空值，s2的值 move到s3上（只是复制了 长度、容量、指针）

    println!("{0}, {1}", s2, s4);
}

fn test_copy(){
/*c++中，有堆区开辟的属性，一定要提供拷贝构造函数防止浅拷贝带来的问题*/
/*rust设计原则：rust永远不会自动地创建数据的深拷贝*/
/*
浅拷贝 copy（move）
深拷贝 clone
*/

let s1 = String::from("hello");
let s2 = s1.clone();
println!("{}, {}", s1, s2);

}

fn test_drop(){
    /*drop跟copy互斥*/
    /*
    如果一个类型拥有了copy这种trait，那么它的变量可以在赋值给其他变量之后任然保持可用性
    如果一个类型本身或者这种类型的任意成员实现了drop这种trait，那么rust就不允许它实现copy这种trait

    caution：任何简单标量的组合类型都是可以copy的（bool float 整形）；任何需要分配内存或者某种资源的都不是copy的
    */

    // 所有权与函数
    // 函数在返回值的过程中也会发生所有权的转移
    let s1 = gives_ownership();

    let s2 = String::from("good morning");
    let s3 = takes_and_gives_back(s2); // 此时s2不再有效
    println!("{}, {}", s1, s3);

    let s4 = 100; // i32类型是copy的，在这里之后还可以继续使用s4
    make_copy(s4);

    /*
        一个变量离开作用域时会被drop函数还回，除非它的所有权转移到另外一个变量上
    */
}

fn gives_ownership()->String{
    let some_string = String::from("hello world");
    some_string // 所有权返回给上面的s1上
}

fn takes_and_gives_back(a_string:String)->String{ // s2的所有权给了函数入参
    a_string // 函数入参的所有权给了函数返回值也就是s3上面
}

fn make_copy(x:i32){
    println!("{}", x); // x离开作用域会正常消亡
}

fn main(){
    add_function();
    let x = add(2,9);
    println!("{}", x);

    if_else();

    match_test();

    test_if();

    loop_test();

    test_while();

    test_for();

    test_range();

    test_owner();

    test_string();

    test_copy();

    test_drop();
}