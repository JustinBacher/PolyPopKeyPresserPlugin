Instance.properties = properties({
	{ name="Action", type="Enum", items={
		"Up",
		"Down",
		"Left",
		"Right",
	} },
	{ name="Amount", type="Int" }
})

function Instance:getAction()
	return '"scroll" "' .. self.properties.Action .. '" "' .. self.properties.Amount .. '"'
end