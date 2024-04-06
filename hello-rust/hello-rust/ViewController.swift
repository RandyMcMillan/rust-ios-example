import UIKit
class ViewController: UIViewController {
        
    @IBOutlet weak var textView: UITextView!

     override func viewDidLoad() {
         super.viewDidLoad()

         let result = rust_hello("world<-from-result")
         let swift_result = String(cString: result!)
         rust_hello_free(UnsafeMutablePointer(mutating: result))
         print(swift_result)
         self.textView.text = swift_result.lowercased()

     }

     override func didReceiveMemoryWarning() {
         super.didReceiveMemoryWarning()
         // Dispose of any resources that can be recreated.
     }

}
