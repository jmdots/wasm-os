const PAGE_SIZE: usize = 4096;
const MAX_PAGES: usize = 256;

pub struct MemoryManager {
    pages: Vec<Page>,
    free_pages: Vec<usize>,
}

struct Page {
    in_use: bool,
    process_id: Option<u32>,
    data: Vec<u8>,
}

impl MemoryManager {
    pub fn new() -> Self {
        let mut pages = Vec::with_capacity(MAX_PAGES);
        let mut free_pages = Vec::with_capacity(MAX_PAGES);

        for i in 0..MAX_PAGES {
            pages.push(Page {
                in_use: false,
                process_id: None,
                data: vec![0; PAGE_SIZE],
            });
            free_pages.push(i);
        }

        Self { pages, free_pages }
    }

    pub fn allocate_page(&mut self, process_id: u32) -> Option<usize> {
        if let Some(page_index) = self.free_pages.pop() {
            let page = &mut self.pages[page_index];
            page.in_use = true;
            page.process_id = Some(process_id);
            Some(page_index)
        } else {
            None
        }
    }

    pub fn free_page(&mut self, page_index: usize) -> Result<(), &'static str> {
        if page_index >= self.pages.len() {
            return Err("Invalid page index");
        }

        let page = &mut self.pages[page_index];
        if !page.in_use {
            return Err("Page already free");
        }

        page.in_use = false;
        page.process_id = None;
        page.data.fill(0);
        self.free_pages.push(page_index);

        Ok(())
    }

    pub fn write_to_page(&mut self, page_index: usize, offset: usize, data: &[u8]) -> Result<(), &'static str> {
        if page_index >= self.pages.len() {
            return Err("Invalid page index");
        }

        let page = &mut self.pages[page_index];
        if !page.in_use {
            return Err("Page not allocated");
        }

        if offset + data.len() > PAGE_SIZE {
            return Err("Write would exceed page bounds");
        }

        page.data[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }
}