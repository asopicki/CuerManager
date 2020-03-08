import { Component, Inject, OnInit, AfterContentInit } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';

import tuiEditor from 'tui-editor';

export interface NotesDialogData {
  notes: string;
}

@Component({
  selector: 'app-notes-editor',
  templateUrl: './notes-editor.component.html',
  styleUrls: ['./notes-editor.component.scss']
})
export class NotesEditorComponent implements OnInit, AfterContentInit {

  private editor: tuiEditor;
  
  constructor(public dialogRef: MatDialogRef<NotesEditorComponent>, @Inject(MAT_DIALOG_DATA) public data: NotesDialogData) { 
    
  }

  ngOnInit() {
    let elem =  document.getElementById('tuiEditor');

    if (elem) {
      this.editor = new tuiEditor({
        el: document.getElementById('tuiEditor'),
        initialEditType: 'wysiwyg',
        height: '550px',
        usageStatistics: false
      });
      this.editor.setMarkdown(this.data.notes);
    }
  }

  ngAfterContentInit() {
    

  }

  onCancelClick(): void {
    this.dialogRef.close();
  }

  onSave(data): NotesDialogData {
    if (this.editor) {
      data.notes = this.editor.getMarkdown().replace("\n<br>", "\r\n");
    }
    return data;
  }
}
