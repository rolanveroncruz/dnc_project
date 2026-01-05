import {Component, Inject} from '@angular/core';
import {JsonPipe} from "@angular/common";
import {MatButton} from "@angular/material/button";
import {
  MAT_DIALOG_DATA,
  MatDialogActions,
  MatDialogContent,
  MatDialogRef,
  MatDialogTitle
} from "@angular/material/dialog";

@Component({
  selector: 'app-add-edit-user',
    imports: [
        JsonPipe,
        MatButton,
        MatDialogActions,
        MatDialogContent,
        MatDialogTitle
    ],
  templateUrl: './add-edit-user.html',
  styleUrl: './add-edit-user.scss',
})
export class AddEditUser {
  constructor(@Inject(MAT_DIALOG_DATA) public data:any,
              private ref: MatDialogRef<AddEditUser>){}

  close() {
    this.ref.close();
  }

  save() {
    this.ref.close(this.data);
  }
}
