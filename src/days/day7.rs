use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day7_test.txt"), 95437);
    dbg!(part1("assets/day7.txt"));
    assert_eq!(part2("assets/day7_test.txt"), 24933642);
    dbg!(part2("assets/day7.txt"));
}

fn part2(file: &str) -> usize {
    let filesystem = parse(file);
    let directory_sizes = filesystem.directory_sizes();
    let available_size = 70000000 - filesystem.size();
    directory_sizes.values().filter(|x| **x > (30000000 - available_size)).min().copied().unwrap_or(0)
}

fn part1(file: &str) -> usize {
    let filesystem = parse(file);
    let directory_sizes = filesystem.directory_sizes();
    directory_sizes.values().filter(|x| **x <= 100000).sum()
}

fn parse(file: &str) -> Content {
    let mut root = Content {
        name: String::from("/"),
        path: Vec::new(),
        content: ContentKind::Directory(HashMap::new()),
    };
    let mut cur_view = Vec::new();
    for line in read_file(file).lines() {
        if line.starts_with('$') {
            let args = line.split(' ').collect::<Vec<_>>();
            match args[1] {
                "cd" => match args[2] {
                    "/" => cur_view = Vec::new(),
                    ".." => {cur_view.pop();},
                    dir => cur_view.push(String::from(dir)),
                },
                _ => {},
            }
        } else {
            let contents = line.split(' ').collect::<Vec<_>>();
            let kind = match contents[0] {
                "dir" => ContentKind::Directory(HashMap::new()),
                size => ContentKind::File(size.parse::<usize>().expect("Yeet")),
            };
            let cur_view = cur_view.iter().map(String::from).collect::<Vec<_>>();
            let content = Content {
                name: String::from(contents[1]),
                path: cur_view.clone(),
                content: kind,
            };
            root.push_content(cur_view, content);
        }
    }
    root
}

#[derive(Debug)]
struct Content {
    name: String,
    path: Vec<String>,
    content: ContentKind,
}

impl Content {
    fn push_content(&mut self, path: Vec<String>, content: Content) {
        if let ContentKind::Directory(contents) = &mut self.get_dir(VecDeque::from(path)).content {
            contents.insert(content.name.clone(), content);
        }
    }

    fn get_dir(&mut self, mut path: VecDeque<String>) -> &mut Self {
        if let Some(dir) = path.pop_front() {
            if let ContentKind::Directory(contents) = &mut self.content {
                contents.get_mut(&dir).expect("Yeet").get_dir(path)
            } else {
                panic!("Yeet")
            }
        } else {
            self
        }
    }

    fn size(&self) -> usize {
        match &self.content {
            ContentKind::File(size) => *size,
            ContentKind::Directory(contents) => contents.values().map(|c| c.size()).sum(),
        }
    }

    fn directory_sizes(&self) -> HashMap<Vec<String>, usize> {
        let mut sizes = HashMap::new();
        if let ContentKind::Directory(contents) = &self.content {
            let mut key = self.path.clone();
            key.push(self.name.clone());
            sizes.insert(key, self.size());
            for content in contents.values() {
                sizes.extend(content.directory_sizes())
            }
        }
        sizes
    }
}

#[derive(Debug)]
enum ContentKind {
    Directory(HashMap<String, Content>),
    File(usize),
}
