Instance.properties = properties({
	{ name="X", type="Int" },
	{ name="Y", type="Int" },
	{ name="Relative", type="Bool" },
})

function Instance:getAction()
	return '"move" "' .. tostring(self.properties.Relative) .. '" "' .. tostring(self.properties.X) '" "'.. tostring(self.properties.Y) .. '"'
end