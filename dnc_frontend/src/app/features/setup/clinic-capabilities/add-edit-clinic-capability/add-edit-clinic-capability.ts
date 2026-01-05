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
  selector: 'app-add-edit-clinic-capability',
    imports: [
        JsonPipe,
        MatButton,
        MatDialogActions,
        MatDialogContent,
        MatDialogTitle
    ],
  templateUrl: './add-edit-clinic-capability.html',
  styleUrl: './add-edit-clinic-capability.scss',
})
export class AddEditClinicCapability {
  constructor(@Inject(MAT_DIALOG_DATA) public data:any,
              private ref: MatDialogRef<AddEditClinicCapability>){}

  close() {
    this.ref.close();
  }

  save() {
    this.ref.close(this.data);
  }
}
