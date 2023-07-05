pub enum ExportPub {
    XPub, 
    YPub,
    ZPub,
}

impl KeyManager for HDKey {
    fn export_to_pub(export_style: ExportPub) {
        todo!()
    }

}