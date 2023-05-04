local mouseController = getLocalFolder() .. "mouse_controller.exe "

Instance.properties = properties({
	{ name="Click", type="PropertyGroup", items={
		{ name="Button", type="Enum", items={
			"Left",
			"Middle",
			"Right",
			"Back",
			"Forward",
		} },
		{ name="Duration", type="Int", units="Seconds", range={min=0}, ui={easing=10} },
		{ name="Click", type="Action",  }
	} },
	{ name="Move", type="PropertyGroup", items={
		{ name="X", type="Int" },
		{ name="Y", type="Int" },
		{ name="Relative", type="Bool", value=true },
		{ name="Duration", type="Int", units="ms", value=0, range={min=0,max=10000}, ui={easing=500} },
		{ name="MoveMouse", type="Action",  }
	} },
	{ name="Scroll", type="PropertyGroup", items={
		{ name="Direction", type="Enum", items={
			"Up",
			"Down",
			"Left",
			"Right",
		} },
		{ name="Amount", type="Int", range={min=0}, ui={easing=10} },
		{ name="Scroll", type="Action" }
	} },
})

function Instance:Click()
	local button = self.properties.Click.Button
	local duration = self.properties.Click.Duration

	getOS():run(
		"Mouse" .. button.. " clicked for " .. duration .. "(s)",
		mouseController .. '"click" "' .. button .. '" "' .. duration .. '"'
	)
end

function Instance:MoveMouse()
	local x, y = self.properties.Move.X, self.properties.Move.Y
	local relative = self.properties.Move.Relative
	local relativeText = "."

	if relative then relativeText = " relative to current position." end

	getOS():run(
		"Mouse Moved [" .. x .. ", " .. y .. "]" .. relativeText,
		mouseController .. '"move" "' .. x .. '" "' .. y .. '" "' .. tostring(relative) .. '" "' .. self.properties.Move.Duration .. '"'
	)
end

function Instance:Scroll()
	local direction = self.properties.Scroll.Direction
	local units = self.properties.Scroll.Amount

	getOS():run(
		"Mouse Scrolled " .. direction .. " " .. units .. " units",
		mouseController .. '"scroll" "' .. units .. '" "' .. direction .. '"'
	)
end