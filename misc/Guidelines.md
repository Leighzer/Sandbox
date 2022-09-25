# Guidelines

Reduce Lines of Code - Remove zero reference functions and variables. Less lines of code to revie and maintain means less work. Use it or lose it.

Data can just be data - Not all classes have to have methods. It is ok to make classes just to bundle values in a convenient package.

Follow conventions for your project and environment - Follow established patterns (formatting, variable naming, directory structure etc.) in your project over mixing styles. Also adhere to similar conventions for your environment. This makes code easier to review and refactor for yourself and others.

Modified clients should present problems only to the client user - For client/server applications, rogue users modifying clients should only present inconveniences for themselves. The server should separately validate and enforce all rules around client requests. For example if a user modifies their client and breaks their ability to push buttons, that is something you do not need to worry about. What you do need to worry about is if a rogue client is able to negatively impact the server and other client applications. Client side validation is only a convenience to the user experience. Server side validation is required to actually prevent unwanted/not allowed behavior and enforce functional requirements.