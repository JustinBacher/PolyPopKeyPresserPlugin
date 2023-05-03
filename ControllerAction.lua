Instance.properties = properties({
	{ name="Action", type="Enum", items={
		"None",
		"A",
		"B",
		"X",
		"Y",
		"Menu",
		"View",
		"D Pad Down",
		"D Pad Left",
		"D Pad Right",
		"D Pad Up",
		"Left Shoulder",
		"Left Trigger",
		"Right Shoulder",
		"Right Trigger",
		"Left Thumbstick Button",
		"Left Thumbstick Down",
		"Left Thumbstick Left",
		"Left Thumbstick Right",
		"Left Thumbstick Up",
		"Right Thumbstick Button",
		"Right Thumbstick Down",
		"Right Thumbstick Left",
		"Right Thumbstick Right",
		"Right Thumbstick Up",
	}, onUpdate="onActionUpdate" }
})

function Instance:getAction()
	return "GamePad" .. self.properties.Action:gsub("%s+", "")
end

function Instance:onActionUpdate()
	self:setName(self.properties.Action)
end
