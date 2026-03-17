import {CommonModule} from '@angular/common';
import { Component,Inject } from '@angular/core';
import{MAT_DIALOG_DATA, MatDialogRef, MatDialogModule} from "@angular/material/dialog";
import {MatButtonModule} from "@angular/material/button";

export interface SimpleConfirmDialogData {

    title: string;
    message: string;
    confirmText?: string;
    cancelText?: string;
}
@Component({
    selector: 'app-simple-confirm-dialog-component',
    standalone: true,
    imports: [CommonModule, MatDialogModule, MatButtonModule],
    templateUrl: './simple-confirm-dialog-component.html',
    styleUrl: './simple-confirm-dialog-component.scss',
})
export class SimpleConfirmDialogComponent {
    constructor(
        private dialogRef: MatDialogRef<SimpleConfirmDialogComponent>,
        @Inject(MAT_DIALOG_DATA) public data: SimpleConfirmDialogData,
    ) {}

    close(result: boolean) {
        this.dialogRef.close(result);
    }
}
