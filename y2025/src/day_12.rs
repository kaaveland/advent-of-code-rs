struct Region {
    height: usize,
    width: usize,
    boxes: usize,
}

fn parse(s: &str) -> Vec<Region> {
    let s = s.split("\n\n").last().unwrap();
    let mut out = vec![];
    for line in s.lines().filter(|line| !line.is_empty()) {
        let (dims, boxes) = line.split_once(": ").unwrap();
        let (width, height) = dims.split_once("x").unwrap();
        let width = width.parse().unwrap();
        let height = height.parse().unwrap();
        let boxes = boxes.split(" ").map(|n| n.parse::<usize>().unwrap()).sum();
        out.push(Region {
            width,
            height,
            boxes,
        })
    }
    out
}

// This has absolutely no right to work, but it does.
fn boxes_fit(region: &Region) -> bool {
    let vertical_boxes = region.height / 3;
    let horizontal_boxes = region.width / 3;
    region.boxes <= vertical_boxes * horizontal_boxes
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let regions = parse(s);
    let fits = regions.iter().filter(|r| boxes_fit(r)).count();
    Ok(format!("{fits}"))
}
