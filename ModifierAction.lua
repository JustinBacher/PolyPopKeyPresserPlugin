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
	} }
})

function Instance:getAction()
	return self.properties.Action
end
