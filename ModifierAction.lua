Instance.properties = properties({
	{ name="Modifier", type="Enum", items={
		"None",
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

function Instance:onInit()
	getUI():setUIProperty({{obj=self, expand=true}})
end

function Instance:getAction()
	return self.properties.Modifier
end

function Instance:onActionUpdate()
	self:setName(self.properties.Modifier)
end
