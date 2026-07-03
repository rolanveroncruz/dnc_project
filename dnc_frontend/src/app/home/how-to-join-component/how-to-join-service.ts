import { inject,Injectable } from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {environment} from '../../../environments/environment';

export interface SubmitDentistApplicationResponse {
    id: number;
    message: string;
}
// ✅ NEW: fixed ownership values matching backend/database CHECK constraint
export type ClinicOwnershipType =
    | 'single_proprietorship'
    | 'company'
    | 'corporation';

// ✅ NEW: typed frontend representation of the text form
export interface DentistApplicationFormValue {
    name: string;
    clinic_name: string;
    contact_numbers: string;
    email: string;
    clinic_ownership_type: ClinicOwnershipType;
    hmo_affiliations: string;
    clinic_address: string;
}

// ✅ NEW: typed frontend representation of uploaded files
export interface DentistApplicationFiles {
    prcLicenseFile: File;
    bir2303File: File;
    registrationDocFile: File;
    supportingDocsFile1?: File | null;
}


@Injectable({
  providedIn: 'root',
})
export class HowToJoinService {
    private http = inject(HttpClient);
    private baseUrl = environment.apiUrl;


    submitDentistApplication(
        formValue: DentistApplicationFormValue,
        files: DentistApplicationFiles
    ):Observable<SubmitDentistApplicationResponse>{
        const formData = new FormData();
        formData.append('name', formValue.name);
        formData.append('clinic_name', formValue.clinic_name);
        formData.append('contact_numbers', formValue.contact_numbers);
        formData.append('email', formValue.email);
        formData.append('clinic_ownership_type', formValue.clinic_ownership_type);
        formData.append('hmo_affiliations', formValue.hmo_affiliations);
        formData.append('clinic_address', formValue.clinic_address);

        formData.append('prc_license_file', files.prcLicenseFile);
        formData.append('bir_2303_file', files.bir2303File);
        formData.append('registration_doc_file', files.registrationDocFile);
        if (files.supportingDocsFile1) {
            formData.append('supporting_docs_file1', files.supportingDocsFile1);
        }


        return this.http.post<SubmitDentistApplicationResponse>(
            `${this.baseUrl}/public/dentist_applications`,
            formData,
        );
    }
}
