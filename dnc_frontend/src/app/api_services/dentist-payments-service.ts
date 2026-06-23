import {inject, Injectable} from '@angular/core';
import {environment} from '../../environments/environment';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {LoginService} from '../login.service';
import {Observable} from 'rxjs';

export interface DentistPaymentMatrixResponse {
    months: DentistPaymentMatrixMonth[];
    rows: DentistPaymentMatrixRow[];
}

export interface DentistPaymentMatrixMonth {
    year: number;
    month: number;
    label: string;
}

export interface DentistPaymentMatrixRow {
    dentist_clinic_id: number;
    dentist_id: number;
    dentist_name: string;
    clinic_id: number;
    clinic_name: string;
    cells: DentistPaymentMatrixCell[];
}

export interface DentistPaymentMatrixCell {
    year: number;
    month: number;
    paid: boolean;
    payment_id: number | null;
    report_name: string | null;
    date_paid: string | null;
    date_paid_recorded_by: string | null;
}

@Injectable({
  providedIn: 'root',
})
export class DentistPaymentsService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    private readonly apiUrl = environment.apiUrl;
    private readonly base = `${this.apiUrl}/api/dentists/payments`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    };
    getDentistPaymentsMatrix(): Observable<DentistPaymentMatrixResponse>{
        return this.http.get<DentistPaymentMatrixResponse>(`${this.base}/matrix`,{
            headers: this.authHeaders(),
        });
    }


}
