Instance.properties = properties({
	{ name="Key", type="Enum", items={
		"None",
		"Backspace",
		"CapsLock",
		"Delete",
		"DownArrow",
		"UpArrow",
		"RightArrow",
		"LeftArrow",
		"Home",
    	"End",
		"Insert",
		"PageDown",
		"PageUp",
		"Pause",
		"Print",
		"Escape",
		"Space",
		"Tab",
		"F1",
		"F2",
		"F3",
		"F4",
		"F5",
		"F6",
		"F7",
		"F8",
		"F9",
		"F10",
		"F11",
		"F12",
		"F13",
		"F14",
		"F15",
		"F16",
		"F17",
		"F18",
		"F19",
		"F20",
		"F21",
		"F22",
		"F23",
		"F24",
	 }, onUpdate="onActionUpdate" }
})

function Instance:onInit()
	getUI():setUIProperty({{obj=self, expand=true}})
end

function Instance:getAction()
	return self.properties.Key
end

function Instance:onActionUpdate()
	self:setName(self.properties.Key)
end

