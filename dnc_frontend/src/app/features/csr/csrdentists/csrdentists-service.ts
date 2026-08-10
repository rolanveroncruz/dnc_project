import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {LoginService} from '../../../login.service';
import {environment} from '../../../../environments/environment';
import {Observable} from 'rxjs';
export interface ClinicCapabilityResponse {
    id: number;
    name: string;
    active: boolean;
}

export interface DentistClinicResponse {
    id: number;
    name: string;
    address: string;
    contact_number: string;
    schedule: string;
    capabilities: ClinicCapabilityResponse[];
}

export interface DentistCompanyResponse {
    id: number;
    name: string;
}

export interface DentistHmoResponse {
    id: number;
    name: string;
}

export interface Dentist {
    id: number;

    prc_no: string | null;
    prc_expiry_date: string | null;

    last_name: string;
    given_name: string;
    middle_name: string | null;

    email: string | null;
    notes: string | null;

    retainer_fee: number;

    dentist_status_id: number | null;
    dentist_decline_remarks: string | null;
    dentist_history_id: number | null;
    dentist_requested_by: string | null;

    accre_dentist_contract_id: number | null;
    accre_document_code: string | null;
    accreditation_date: string | null;
    accre_contract_sent_date: string | null;
    accre_contract_file_path: string | null;
}

export interface DentistWithClinicsResponse extends Dentist {
    clinics: DentistClinicResponse[];

    exclusive_to_companies: DentistCompanyResponse[];
    except_for_companies: DentistCompanyResponse[];

    exclusive_to_hmos: DentistHmoResponse[];
    except_for_hmos: DentistHmoResponse[];
}

@Injectable({
  providedIn: 'root',
})
export class CSRDentistsService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);
    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }

    // ANNOTATED CHANGE: adjust this to match your actual route
    private readonly baseUrl = `${environment.apiUrl}/api/csr/dentists`;

    getDentists():Observable<DentistWithClinicsResponse[]>{
        return this.http.get<DentistWithClinicsResponse[]>(
            this.baseUrl,
            {headers: this.authHeaders(),}
        );
    }

}
