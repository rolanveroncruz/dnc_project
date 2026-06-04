import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {LoginService} from '../login.service';
import {environment} from '../../environments/environment';
import {Observable} from 'rxjs';

export interface DentistClinicReconciledJobsPivotRow {
    id: number | null;
    dentist_name: string | null;
    clinic_name: string | null;
    position_name: string | null;
    contract_name: string | null;

    // Dynamic month columns:
    // e.g. "Jun 2025": 10, "Jul 2025": 8
    [key: string]: string | number | null;
}

@Injectable({
  providedIn: 'root',
})
export class DentistRetainerFeesService {
    private http = inject(HttpClient);
    private loginService = inject(LoginService);

    // Adjust this if your backend uses a different base path
    // (matches your existing services style)
    private readonly baseUrl = `${environment.apiUrl}`;
    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }

    getDentistClinicReconciledJobsLast12Months(): Observable<DentistClinicReconciledJobsPivotRow[]>{
        return this.http.get<DentistClinicReconciledJobsPivotRow[]>(`${this.baseUrl}/api/dentist_clinics/reconciled_jobs`,
            { headers: this.authHeaders() });
    }

}
