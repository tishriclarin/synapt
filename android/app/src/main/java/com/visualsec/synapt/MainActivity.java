package com.visualsec.synapt;
import android.app.Activity;
import android.os.Bundle;
import android.widget.*;
public class MainActivity extends Activity {
 static { System.loadLibrary("synapt"); }
 private native String nativeVersion();
 private LinearLayout root;
 private void reset(){root=new LinearLayout(this);root.setOrientation(LinearLayout.VERTICAL);root.setPadding(24,24,24,24);setContentView(root);}
 private void label(String text){TextView v=new TextView(this);v.setText(text);v.setTextSize(18);root.addView(v);}
 private void button(String text,Runnable action){Button b=new Button(this);b.setText(text);b.setOnClickListener(v->action.run());root.addView(b);}
 @Override public void onCreate(Bundle state){super.onCreate(state);chat();}
 private void chat(){reset();button("☰",this::menu);label("SynapT");TextView status=new TextView(this);status.setText("Chat provider not connected. Native core "+nativeVersion());root.addView(status,new LinearLayout.LayoutParams(-1,0,1));EditText input=new EditText(this);input.setHint("Message");root.addView(input);button("Send",()->Toast.makeText(this,"No supported chat provider connected",Toast.LENGTH_SHORT).show());}
 private void menu(){reset();button("Back",this::chat);label("SynapT");button("Files",this::files);button("Chat",this::chat);}
 private void files(){reset();button("Back",this::menu);label("Files");EditText search=new EditText(this);search.setHint("Search files");root.addView(search);label("No connected file provider");}
}
