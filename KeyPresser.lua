local keyPresser = getLocalFolder() .. "key_presser/target/release/key_presser.exe "

Instance.properties = properties({
	{ name="Actions", type="ObjectSet", set_types={type="PolyPopObject", index="KeyPresser.KeyAction"}, ui={expand=true} },
	{ name="Duration", type="Int", units="ms", range={min=0}, ui={easing=10} },
	{ name="Press", type="Action" }
})

function Instance:onInit()
	getUI():setUIProperty({{obj=self, expand=true}})
end

function Instance:Press()
	local actions = ""
	local duration = self.properties.Duration

	for i=1, self.properties.Actions:getKit():getObjectCount() do
		local action = self.properties.Actions:getKit():getObjectByIndex(i):getAction()

		if action ~= "None" then
			actions = actions .. ' "' .. action .. '"'
		end
	end
	print(keyPresser .. '"' .. duration .. '"' .. actions)
	if actions ~= "" then
		getOS():run(
			"Key(s) Pressed: [" .. actions .. "] for " .. duration .. "(s)",
			keyPresser .. '"' .. duration .. '"' .. actions
		)
	end
end



