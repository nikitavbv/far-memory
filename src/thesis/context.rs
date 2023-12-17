use {
    std::collections::HashMap,
    crate::thesis::engine::Reference,
};

pub struct Context {
    numbering_id_counter: usize,
    sections: Vec<SectionContext>,
    references: HashMap<String, u32>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            numbering_id_counter: 0,
            sections: Vec::new(),
            references: HashMap::new(),
        }
    }

    pub fn next_numbering_id(&mut self) -> usize {
        self.numbering_id_counter += 1;
        self.numbering_id_counter
    }

    pub fn last_section_index(&self) -> usize {
        self.sections.len()
    }

    pub fn next_section_index(&mut self) -> usize {
        self.sections.push(SectionContext::new());
        self.last_section_index()
    }

    fn section(&mut self, section_index: usize) -> &mut SectionContext {
        // quick fix for images in conference abstract
        if self.sections.is_empty() {
            self.sections.push(SectionContext::new());
        }

        // quick fix for images in conference abstract
        let section_index = if section_index == 0 {
            0
        } else {
            section_index - 1
        };

        self.sections.get_mut(section_index).unwrap()
    }

    pub fn next_subsection_index(&mut self, section_index: usize, level: u32) -> SectionIndex {
        self.section(section_index).next_subsection_index(level)
    }

    pub fn next_image_index(&mut self, section_index: usize) -> usize {
        self.section(section_index).next_image_index()
    }

    pub fn reference_id_for(&mut self, reference: &Reference) -> u32 {
        let reference_text = reference.text();
        if let Some(id) = self.references.get(reference_text) {
            return *id;
        } else {
            let next_id = (self.references.len() + 1) as u32;
            self.references.insert(reference_text.to_owned(), next_id);
            next_id
        }
    }
}

struct SectionContext {
    subsection_counter: SectionIndex,
    image_counter: usize,
}

impl SectionContext {
    pub fn new() -> Self {
        Self {
            subsection_counter: SectionIndex::new(),
            image_counter: 0,
        }
    }

    pub fn next_subsection_index(&mut self, level: u32) -> SectionIndex {
        self.subsection_counter = self.subsection_counter.clone().next_at_level(level);
        self.subsection_counter.clone()
    }

    pub fn next_image_index(&mut self) -> usize {
        self.image_counter += 1;
        self.image_counter
    }
}

#[derive(Clone)]
pub struct SectionIndex {
    index: Vec<usize>,
}

impl SectionIndex {
    pub fn new() -> Self {
        Self {
            index: Vec::new(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = "".to_owned();

        for i in 0..self.index.len() {
            if i == 0 {
               result = self.index[i].to_string();
            } else {
                result = format!("{}.{}", result, self.index[i]);
            }
        }

        result
    }

    pub fn next_at_level(self, level: u32) -> Self {
        let mut index = self.index;

        while index.len() < level as usize {
            index.push(if index.len() == level as usize - 1 { 0 } else { 1 });
        }

        index[level as usize - 1] += 1;
        while index.len() > level as usize {
            index.pop();
        }

        Self {
            index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_index_simple() {
        let index = SectionIndex::new();
        let index = index.next_at_level(1);
        assert_eq!("1".to_owned(), index.to_string());

        let index = index.next_at_level(1);
        assert_eq!("2".to_owned(), index.to_string());

        let index = index.next_at_level(2);
        assert_eq!("2.1".to_owned(), index.to_string());

        let index = index.next_at_level(2);
        assert_eq!("2.2".to_owned(), index.to_string());

        let index = index.next_at_level(1);
        assert_eq!("3".to_owned(), index.to_string());

        let index = index.next_at_level(2);
        assert_eq!("3.1".to_owned(), index.to_string());
    }
}
