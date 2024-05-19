from burp import IBurpExtender, IContextMenuFactory
from javax.swing import JMenuItem


class BurpExtender(IBurpExtender, IContextMenuFactory):
  def registerExtenderCallbacks(self, callbacks):
    self._callbacks = callbacks
    self._helpers = callbacks.getHelpers()
    callbacks.setExtensionName("F1-Intruder")
    callbacks.registerContextMenuFactory(self)
    print( "[+] F1 extension loaded in" )
    return

  def createMenuItems(self, invocation):
    menu_item = JMenuItem("Send request to F1-Intruder")

    if invocation.getInvocationContext() != 0:
        return []

    def sendToF1(event):
      request = invocation.getSelectedMessages()[0].getRequest().tostring()
      file = open("/tmp/request.data", 'w')
      file.write(request)
      file.close()
      print(request)
      return
        
    menu_item.addActionListener(sendToF1)
    return [menu_item]

