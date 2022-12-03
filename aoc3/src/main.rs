use std::fs;

struct Alphabet{
    a:u32,
    b:u32,
    c:u32,
    d:u32,
    e:u32,
    f:u32,
    g:u32,
    h:u32,
    i:u32,
    j:u32,
    k:u32,
    l:u32,
    m:u32,
    n:u32,
    o:u32,
    p:u32,
    q:u32,
    r:u32,
    s:u32,
    t:u32,
    u:u32,
    v:u32,
    w:u32,
    x:u32,
    y:u32,
    z:u32,
    A:u32,
    B:u32,
    C:u32,
    D:u32,
    E:u32,
    F:u32,
    G:u32,
    H:u32,
    I:u32,
    J:u32,
    K:u32,
    L:u32,
    M:u32,
    N:u32,
    O:u32,
    P:u32,
    Q:u32,
    R:u32,
    S:u32,
    T:u32,
    U:u32,
    V:u32,
    W:u32,
    X:u32,
    Y:u32,
    Z:u32,
}
impl  Alphabet {
    fn get_string(&self, field: &str) -> &u32 {
        match field {
            "a" => &self.a,
            "b" => &self.b,
            "c" => &self.c,
            "d" => &self.d,
            "e" => &self.e,
            "f" => &self.f,
            "g" => &self.g,
            "h" => &self.h,
            "i" => &self.i,
            "j" => &self.j,
            "k" => &self.k,
            "l" => &self.l,
            "m" => &self.m,
            "n" => &self.n,
            "o" => &self.o,
            "p" => &self.p,
            "q" => &self.q,
            "r" => &self.r,
            "s" => &self.s,
            "t" => &self.t,
            "u" => &self.u,
            "v" => &self.v,
            "w" => &self.w,
            "x" => &self.x,
            "y" => &self.y,
            "z" => &self.z,
            "A" => &self.A,
            "B" => &self.B,
            "C" => &self.C,
            "D" => &self.D,
            "E" => &self.E,
            "F" => &self.F,
            "G" => &self.G,
            "H" => &self.H,
            "I" => &self.I,
            "J" => &self.J,
            "K" => &self.K,
            "L" => &self.L,
            "M" => &self.M,
            "N" => &self.N,
            "O" => &self.O,
            "P" => &self.P,
            "Q" => &self.Q,
            "R" => &self.R,
            "S" => &self.S,
            "T" => &self.T,
            "U" => &self.U,
            "V" => &self.V,
            "W" => &self.W,
            "X" => &self.X,
            "Y" => &self.Y,
            "Z" => &self.Z,
            &_ => &0,
        }
    }
}


fn main() {

    let alpha:Alphabet = Alphabet{
        a:1,
        b:2,
        c:3,
        d:4,
        e:5,
        f:6,
        g:7,
        h:8,
        i:9,
        j:10,
        k:11,
        l:12,
        m:13,
        n:14,
        o:15,
        p:16,
        q:17,
        r:18,
        s:19,
        t:20,
        u:21,
        v:22,
        w:23,
        x:24,
        y:25,
        z:26,
        A:27,
        B:28,
        C:29,
        D:30,
        E:31,
        F:32,
        G:33,
        H:34,
        I:35,
        J:36,
        K:37,
        L:38,
        M:39,
        N:40,
        O:41,
        P:42,
        Q:43,
        R:44,
        S:45,
        T:46,
        U:47,
        V:48,
        W:49,
        X:50,
        Y:51,
        Z:52,
    };
    let input:String = fs::read_to_string("./src/input.txt").expect("file should be read");
    let mut total_priority:u32 = 0;

    let mut elf1: &str;
    let mut elf2: &str;
    let mut elf3: &str;

    for pack in input.split("\n"){

        let mut i:u16 = 3;

        while i > 0 {
            
            

            i -= 1;
        }

        for item in second.chars(){

            let mut matches: Vec<&str> = first.matches(item).collect();

            if !matches.is_empty(){
                let match_item:&str = matches.pop().unwrap();
                let num: &u32 = alpha.get_string(match_item);
                total_priority += num;
                break;
            }

        }
    }

    println!("{total_priority}")
}
