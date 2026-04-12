import {CommonModule} from '@angular/common';
import {Component, Inject} from '@angular/core';
import {
    FormControl,
    FormGroup,
    ReactiveFormsModule,
    ValidationErrors,
    AbstractControl,
} from '@angular/forms';
import {
    MAT_DIALOG_DATA,
    MatDialogModule,
    MatDialogRef,
} from '@angular/material/dialog';
import {MatButtonModule} from '@angular/material/button';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {HighEndFilesService} from '../../../../api_services/high-end-files-service';
import {finalize} from 'rxjs';

export interface UploadHighEndServiceFilesDialogData {
    verification_id: number;
    date: string | Date;
    dentist_id: number;
    dentist_name: string;
    master_list_member_id: number;
    master_list_member_name: string;
    dental_service_id: number;
    dental_service_name: string;
}

export interface UploadHighEndServiceFilesDialogResult {
    confirmed: boolean;
    file: File | null;
    description: string | null;
}

function fileRequiredValidator(control: AbstractControl): ValidationErrors | null {
    const file = control.value as File | null;
    return file ? null : {requiredFile: true};
}


@Component({
    selector: 'app-upload-high-end-service-files',
    standalone: true,
    imports: [
        CommonModule,
        ReactiveFormsModule,
        MatDialogModule,
        MatButtonModule,
        MatFormFieldModule,
        MatInputModule,
    ],
    templateUrl: './upload-high-end-service-files-component.html',
    styleUrl: './upload-high-end-service-files-component.scss',
})
export class UploadHighEndServiceFilesComponent {
    isUploading = false;

    readonly form: FormGroup<{
        file: FormControl<File | null>;
        description: FormControl<string | null>;
    }>;

    constructor(
        private dialogRef: MatDialogRef<
            UploadHighEndServiceFilesComponent,
            UploadHighEndServiceFilesDialogResult
        >,
        @Inject(MAT_DIALOG_DATA) public data: UploadHighEndServiceFilesDialogData,
        private highEndFilesService: HighEndFilesService,
    ) {
        this.form = new FormGroup({
            file: new FormControl<File | null>(null, {
                validators: [fileRequiredValidator]
            }),
            description: new FormControl<string | null>(null),
        });
    }

    get description(): string | null {
        const value = this.form.controls.description.value;
        return value?.trim() ? value.trim() : null;
    }

    get selectedFile(): File | null {
        return this.form.controls.file.value;
    }

    onFilesSelected(event: Event): void {
        const input = event.target as HTMLInputElement;
        const fileList = input.files;

        if (!fileList || fileList.length === 0) {
            this.form.controls.file.setValue(null);
            this.form.controls.file.markAsTouched();
            this.form.controls.file.updateValueAndValidity();
            return;
        }

        const file = fileList[0];
        this.form.controls.file.setValue(file);
        this.form.controls.file.markAsTouched();
        this.form.controls.file.updateValueAndValidity();

        input.value = '';
    }

    clearFile(): void {
        this.form.controls.file.setValue(null);
        this.form.controls.file.markAsTouched();
        this.form.controls.file.updateValueAndValidity();
    }

    cancel(): void {
        this.dialogRef.close({
            confirmed: false,
            file: null,
            description: null,
        });
    }


    submit(): void {
        if (this.form.invalid) {
            this.form.markAllAsTouched();
            return;
        }

        const file = this.selectedFile;
        if (!file) {
            this.form.markAllAsTouched();
            return;
        }

        this.isUploading = true;

        this.highEndFilesService
            .uploadHighEndFile(this.data.verification_id, file, this.description)
            .pipe(
                finalize(() => {
                    this.isUploading = false;
                })
            )
            .subscribe({
                next: () => {
                    this.dialogRef.close({
                        confirmed: true,
                        file,
                        description: this.description,
                    });
                },
                error: (error) => {
                    console.error('Error uploading file:', error);
                },
            });
    }

}
