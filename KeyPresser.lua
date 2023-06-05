Instance.properties = properties({
	{ name="Actions", type="ObjectSet", set_types={type="PolyPopObject", index="KeyPresser.KeyAction"}, ui={expand=true} },
	{ name="Duration", type="Int", units="ms", range={min=0}, ui={easing=10} },
	{ name="Press", type="Action" }
})

function Instance:onInit()
	getUI():setUIProperty({{obj=self, expand=true}})

	local host = getNetwork():getHost("localhost")
    self.port = getNetwork():findOpenPort()
    self.webSocket = host:openWebSocket("ws://localhost:" .. self.port)
	self.webSocket:setAutoReconnect(true)

	getOS():run(
		"key_presser|" .. self.port,
		getLocalFolder() .. "key_presser.exe 127.0.0.1:" .. self.port
	)
end

function Instance:Press()
	local actions = {}

	for i = 1, self.properties.Actions:getKit():getObjectCount() do
		local action = self.properties.Actions:getKit():getObjectByIndex(i):getAction()

		if action ~= "None" then
			table.insert(actions, action)
		end
	end

	self.webSocket:send(json.encode(
		{
			kind = 0,
			duration = self.properties.Duration,
			actions = actions,
		}
	))
end



