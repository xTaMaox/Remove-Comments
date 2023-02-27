impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut open = false;
        let mut ret = vec![];
        let mut cache = String::new();
        
        for s in source {
            let s = s.chars().collect::<Vec<char>>();
            let str = Self::read(&s, 0, &mut open);
            cache += &str;
            if open == false && cache.is_empty() == false {
                ret.push(cache.clone());
                cache.clear();
            }
        }
        
        return ret
    }

    fn read(s: &Vec<char>, k: usize, open: &mut bool) -> String {
        let (mut i, n) = (k, s.len());
        let mut ret = String::new();
        
        if i == n { return ret }
        
        if *open {
            while i + 1 < n {
                if s[i] == '*' && s[i + 1] == '/' {
                    *open = false;
                    return Self::read(s, i + 2, open);
                }
                i += 1;
            }
            
            return ret
        }
        
        while i < n {
            if i + 1 < n && s[i] == '/' && s[i + 1] == '/' { return ret }
            if i + 1 < n && s[i] == '/' && s[i + 1] == '*' {
                *open = true;
                let temp = Self::read(s, i + 2, open);
                ret += &temp;
                return ret
            }
            ret.push(s[i]);
            i += 1;
        }
        
        ret
    }
}