export const Timer = java.util.Timer;
export type Timer = java.util.Timer;

export const TimerTask = java.util.TimerTask;
export type TimerTask = java.util.TimerTask;

export const TimeUnit = java.util.concurrent.TimeUnit;
export type TimeUnit = java.util.concurrent.TimeUnit;

export const System = java.lang.System;
export type System = java.lang.System;

export const Runnable = java.lang.Runnable;
export type Runnable = java.lang.Runnable;
// @NativeClass()
// export class Runnable extends java.lang.Runnable {
//   private action: () => void;

//   private constructor(action: () => void) {
//     super();
//     this.action = action;
//     return global.__native(this);
//   }

//   static create(action: () => void) {
//     return new Runnable(action);
//   }

//   override run(): void {
//     this.action();
//   }
// }


// import io.reactivex.rxjava3.core.Observable;
// import io.reactivex.rxjava3.android.schedulers.AndroidSchedulers;
// import io.reactivex.rxjava3.schedulers.Schedulers;
// import io.reactivex.rxjava3.disposables.CompositeDisposable;

// public class MyActivity extends AppCompatActivity {

//     private TextView resultTextView;
//     private CompositeDisposable disposables = new CompositeDisposable();

//     @Override
//     protected void onCreate(Bundle savedInstanceState) {
//         super.onCreate(savedInstanceState);
//         setContentView(R.layout.activity_main);
//         resultTextView = findViewById(R.id.result_text);
//         Button fetchButton = findViewById(R.id.fetch_button);
        
//         fetchButton.setOnClickListener(v -> fetchData());
//     }

//     private void fetchData() {
//         // 1. Create an Observable that does the background work
//         Observable<List<String>> dataObservable = Observable.fromCallable(() -> {
//             // This runs on the background thread
//             Thread.sleep(2000); // simulate network delay
//             return Arrays.asList("Item 1", "Item 2", "Item 3");
//         });

//         // 2. Subscribe to the Observable with thread control
//         disposables.add(dataObservable
//                 .subscribeOn(Schedulers.io())          // work happens on IO thread
//                 .observeOn(AndroidSchedulers.mainThread()) // results come back on UI thread
//                 .subscribe(
//                         result -> {
//                             // onNext: update UI with the data
//                             resultTextView.setText("Got: " + result.toString());
//                         },
//                         error -> {
//                             // onError: handle error (show toast, log, etc.)
//                             resultTextView.setText("Error: " + error.getMessage());
//                         },
//                         () -> {
//                             // onComplete: (optional) called when all items are emitted
//                         }
//                 ));
//     }

//     @Override
//     protected void onDestroy() {
//         super.onDestroy();
//         // CLEANUP: prevent memory leaks by disposing all subscriptions
//         disposables.clear();
//     }
// }