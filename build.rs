fn main() {
	let mut r = winres::WindowsResource::new();
	r.set_manifest(
		r#"
		<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
		<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
		<security>
			<requestedPrivileges>
				<requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>
			</requestedPrivileges>
		</security>
		</trustInfo>
		</assembly>
		"#
	);
	r.set_icon("icon.ico");
	r.compile().expect("damn, didn't compile.");
}
