Instance.properties = properties({
	{ name="Action", type="Enum", items={
		"Meta",
		"Alt",
		"Control",
		"LControl",
		"RControl",
		"Shift",
		"LShift",
		"RShift",
	}, onUpdate="onActionUpdate" }
})

function Instance:getAction()
	return self.properties.Action
end

function Instance:onActionUpdate()
	self:setName(self.properties.Action)
end
