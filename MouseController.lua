local mouseController = getLocalFolder() .. "mouse_controller/target/release/mouse_controller.exe"

Instance.properties = properties({
	{ name="Click", type="PropertyGroup", items={
		{ name="Action", type="Enum", items={
			"Up",
			"Down",
			"Left",
			"Right",
		} },
		{ name="Amount", type="Int" },
		{ name="Click", type="Action",  }
	} },
	{ name="Move", type="PropertyGroup", items={
		{ name="X", type="Int" },
		{ name="Y", type="Int" },
		{ name="Relative", type="Bool" },
		{ name="moveMouse", type="Action",  }
	} },
	{ name="Scroll", type="PropertyGroup", items={
		{ name="Direction", type="Enum", items={
			"Up",
			"Down",
			"Left",
			"Right",
		} },
		{ name="Amount", type="Int" },
		{ name="Scroll", type="Action" }
	} },
})

function Instance:Scroll()
	local direction = self.properties.Scroll.Direction
	local units = self.properties.Scroll.Amount
	getOS():run(
		"Mouse Scrolled " .. direction .. " " .. units .. " units",
		mouseController .. direction
	)
end

