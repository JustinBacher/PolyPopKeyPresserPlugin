Instance.properties = properties({
	{ name="Action", type="Enum", items={
		"Left",
		"Middle",
		"Right",
		"Back",
		"Forward",
	} }
})

function Instance:getAction()
	local action = self.properties.Action

	if action.find("Scroll") then
		return '"scroll" "' .. tostring(self.properties.Relative) .. '" "' .. tostring(self.properties.X) '" "'.. tostring(self.properties.Y) .. '"'
	else
		return '"click" "' .. tostring(self.properties.Relative) .. '" "' .. tostring(self.properties.X) '" "'.. tostring(self.properties.Y) .. '"'
	end

end