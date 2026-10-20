use std::{fmt, fs, io, path::Path};

use crate::geometry::Vec3f;

/// A triangle mesh. Every face indexes into `vertices`, which `parse_obj`
/// checks, so indexing with a face never goes out of bounds.
pub struct Model {
    vertices: Vec<Vec3f>,
    faces: Vec<[usize; 3]>,
}

#[derive(Debug)]
pub enum ObjError {
    Io(io::Error),
    Malformed {
        line: usize,
        message: String,
    },
    IndexOutOfRange {
        line: usize,
        index: usize,
        vertex_count: usize,
    },
}

/// One line of an OBJ file that the model uses.
enum Record {
    Vertex(Vec3f),
    /// 1-based indices, as written in the file.
    Face([usize; 3]),
}

impl Model {
    pub fn from_obj(path: impl AsRef<Path>) -> Result<Self, ObjError> {
        Self::parse_obj(&fs::read_to_string(path)?)
    }

    /// Reads `v` and `f` lines and ignores everything else. Faces must be
    /// triangles; only the position index of each `v/vt/vn` corner is kept.
    pub fn parse_obj(text: &str) -> Result<Self, ObjError> {
        let records = text
            .lines()
            .enumerate()
            .map(|(i, line)| (i + 1, line))
            .filter_map(|(n, line)| {
                parse_line(line)
                    .map(|r| r.map(|r| (n, r)))
                    .map_err(|message| ObjError::Malformed { line: n, message })
                    .transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let vertices: Vec<Vec3f> = records
            .iter()
            .filter_map(|(_, r)| match r {
                Record::Vertex(v) => Some(*v),
                Record::Face(_) => None,
            })
            .collect();

        let faces = records
            .iter()
            .filter_map(|(line, r)| match r {
                Record::Face(f) => Some((*line, *f)),
                Record::Vertex(_) => None,
            })
            .map(|(line, face)| to_zero_based(line, face, vertices.len()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { vertices, faces })
    }

    pub fn vertices(&self) -> &[Vec3f] {
        &self.vertices
    }

    /// Each face's three vertex positions.
    pub fn faces(&self) -> impl ExactSizeIterator<Item = [Vec3f; 3]> + '_ {
        self.faces.iter().map(|face| face.map(|i| self.vertices[i]))
    }
}

/// `Ok(None)` for lines the model doesn't use. Errors carry a message for the
/// caller to attach the line number to.
fn parse_line(line: &str) -> Result<Option<Record>, String> {
    let mut words = line.split_whitespace();
    match words.next() {
        Some("v") => {
            let [x, y, z] = parse_three(words, |w| w.parse::<f32>().ok())
                .ok_or("expected three numbers after `v`")?;
            Ok(Some(Record::Vertex(Vec3f::new(x, y, z))))
        }
        Some("f") => {
            let corner = |w: &str| w.split('/').next()?.parse::<usize>().ok();
            let face =
                parse_three(words, corner).ok_or("expected three `v/vt/vn` corners after `f`")?;
            Ok(Some(Record::Face(face)))
        }
        _ => Ok(None),
    }
}

/// Exactly three words, each parsed by `parse`.
fn parse_three<'a, T>(
    mut words: impl Iterator<Item = &'a str>,
    parse: impl Fn(&str) -> Option<T>,
) -> Option<[T; 3]> {
    let parsed = [words.next()?, words.next()?, words.next()?].map(&parse);
    match (parsed, words.next()) {
        ([Some(a), Some(b), Some(c)], None) => Some([a, b, c]),
        _ => None,
    }
}

fn to_zero_based(
    line: usize,
    face: [usize; 3],
    vertex_count: usize,
) -> Result<[usize; 3], ObjError> {
    face.iter()
        .find(|&&i| i == 0 || i > vertex_count)
        .map_or(Ok(face.map(|i| i - 1)), |&index| {
            Err(ObjError::IndexOutOfRange {
                line,
                index,
                vertex_count,
            })
        })
}

impl fmt::Display for ObjError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjError::Io(e) => write!(f, "could not read OBJ file: {e}"),
            ObjError::Malformed { line, message } => write!(f, "line {line}: {message}"),
            ObjError::IndexOutOfRange {
                line,
                index,
                vertex_count,
            } => write!(
                f,
                "line {line}: vertex index {index} is outside 1..={vertex_count}"
            ),
        }
    }
}

impl std::error::Error for ObjError {}

impl From<io::Error> for ObjError {
    fn from(e: io::Error) -> Self {
        ObjError::Io(e)
    }
}
