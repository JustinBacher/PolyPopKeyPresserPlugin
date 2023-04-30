local keyPresser = getLocalFolder() .. "key_presser/target/release/key_presser.exe"

Instance.properties = properties({
	{ name="Actions", type="ObjectSet", set_types={type="PolyPopObject", index="KeyPresser.KeyAction"} },
	{ name="Trigger", type="Action" }
})

function Instance:onInit()

end

function Instance:Trigger()
	local actions = ""

	for i=1, self.properties.Actions:getKit():getObjectCount() do
		local action = self.properties.Actions:getKit():getObjectByIndex(i).properties.Action

		if action ~= "None" then
			actions = actions .. ' "' .. action .. '"'
		end
	end

	if actions then
		getOS():run("Key Pressed" .. actions, keyPresser .. actions)
	end
end
